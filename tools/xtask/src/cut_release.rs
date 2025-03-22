use crate::flags::CutRelease;

pub fn cut_release(cut_release: CutRelease) -> anyhow::Result<()> {
    println!("{:?}", cut_release.patch);
    println!("{:?}", cut_release.minor);
    println!("{:?}", cut_release.execute);
    Ok(())
}
