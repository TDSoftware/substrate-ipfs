use std::{borrow::Borrow, path::Path};

use futures::{stream::BoxStream, StreamExt};
use rust_unixfs::walk::{ContinuedWalk, Walker};
use tokio::io::AsyncWriteExt;

use crate::{Ipfs, IpfsPath, IpfsTypes};

use super::UnixfsStatus;

pub async fn get<'a, Types, MaybeOwned, P: AsRef<Path>>(
    ipfs: MaybeOwned,
    path: IpfsPath,
    dest: P,
) -> anyhow::Result<BoxStream<'a, UnixfsStatus>>
where
    Types: IpfsTypes,
    MaybeOwned: Borrow<Ipfs<Types>> + Send + 'a,
{
    let mut file = tokio::fs::File::create(dest).await?;
    let ipfs = ipfs.borrow().clone();

    let (resolved, _) = ipfs.dag().resolve(path.clone(), true).await?;

    let block = resolved.into_unixfs_block()?;

    let cid = block.cid();
    let root_name = block.cid().to_string();

    let mut walker = Walker::new(*cid, root_name);

    let stream = async_stream::stream! {
        let mut cache = None;
        let mut total_size = None;
        let mut written = 0;
        while walker.should_continue() {
            let (next, _) = walker.pending_links();
            let block = match ipfs.get_block(next).await {
                Ok(block) => block,
                Err(e) => {
                    yield UnixfsStatus::FailedStatus { written, total_size, error: Some(anyhow::anyhow!("{e}")) };
                    return;
                } 
            };
            let block_data = block.data();

            match walker.next(block_data, &mut cache) {
                Ok(ContinuedWalk::Bucket(..)) => {}
                Ok(ContinuedWalk::File(segment, _, _, _, size)) => {

                    if segment.is_first() {
                        total_size = Some(size as usize);
                        yield UnixfsStatus::ProgressStatus { written, total_size };
                    }
                    // even if the largest of files can have 256 kB blocks and about the same
                    // amount of content, try to consume it in small parts not to grow the buffers
                    // too much.

                    let mut n = 0usize;
                    let slice = segment.as_ref();
                    let total = slice.len();

                    while n < total {
                        let next = &slice[n..];
                        n += next.len();
                        if let Err(e) = file.write_all(next).await {
                            yield UnixfsStatus::FailedStatus { written, total_size, error: Some(anyhow::anyhow!("{e}")) };
                            return;
                        }
                        if let Err(e) = file.sync_all().await {
                            yield UnixfsStatus::FailedStatus { written, total_size, error: Some(anyhow::anyhow!("{e}")) };
                            return;
                        }

                        written += n;
                        yield UnixfsStatus::ProgressStatus { written, total_size };
                    }

                    if segment.is_last() {
                        yield UnixfsStatus::ProgressStatus { written, total_size };
                    }
                },
                Ok(ContinuedWalk::Directory( .. )) | Ok(ContinuedWalk::RootDirectory( .. )) => {}, //TODO
                Ok(ContinuedWalk::Symlink( .. )) => {},
                Err(e) => {
                    yield UnixfsStatus::FailedStatus { written, total_size, error: Some(anyhow::anyhow!("{e}")) };
                    return;
                }
            };
        };

        yield UnixfsStatus::CompletedStatus { path, written, total_size };
    };

    Ok(stream.boxed())
}
