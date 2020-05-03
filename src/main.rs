use futures::{
    channel::mpsc,
    stream::{StreamExt, TryStreamExt},
};

use indicatif::ProgressBar;

use async_std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp =
        reqwest::get("http://datashat.net/music_for_programming_58-olive_is_the_sun.mp3").await?;

    let total_len = resp.content_length().unwrap();
    let (tx, mut rx) = mpsc::channel(1);

    let stream_fut = resp
        .bytes_stream()
        //.map_err(|e| e.into::<Box<dyn std::error::Error>>())
        .map_err::<Box<dyn std::error::Error>, _>(|e| e.into())
        .try_for_each(|res| {
            let mut tx4fut = tx.clone();
            async move {
                let bytes = res;

                tx4fut.try_send(bytes.len())?;

                Ok(())
            }
        });


    let progress_fut = async move {
        let pb = ProgressBar::new(total_len);

        let mut cur_len = 0;

        while cur_len < total_len {
            if let Some(delta) = rx.try_next()? {
                pb.inc(delta as u64);
                cur_len += delta as u64;
            }
        }


        Result::<(), Box<dyn std::error::Error>>::Ok(())
    };

    let (res1, res2) = futures::future::join(stream_fut, progress_fut).await;
    res1?;
    res2?;

    Ok(())
}
