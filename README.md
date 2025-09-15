# stegano-util
## 一个隐写文件到图片的rust crate

## 目前支持以下隐写方式：
- `luma_lsb_hide`：基于YCbCr中的Luma向量的lsb隐写

示例：
```
use stegano_util::luma_lsb_hide;
use stegano_util::luma_lsb_read;

#[test]
fn test_luma_lsb_hide(){
    let img_path:&str="./tests/input.jpg";
    let less_bin_path:&str="./tests/less.bin";
    let more_bin_path:&str="./tests/more.bin";

    let res1=luma_lsb_hide(img_path, less_bin_path);
    match res1 {
        Ok(_)=>{println!("隐写成功!");},
        Err(err)=>{println!("{}",&err);}
    }
    let res2=luma_lsb_hide(img_path, more_bin_path);
    match res2 {
        Ok(_)=>{println!("隐写成功!");},
        Err(err)=>{println!("{}",&err);}
    }
    
}
#[test]
fn test_luma_lsb_read(){
    let output_path="./output.png";
    let res3=luma_lsb_read(output_path);
    match res3 {
        Ok(value)=>{println!("{:?}",value.iter().map(|b| format!("{:02x}", b)).collect::<String>());},
        Err(err)=>{
            println!("{}",err);
            return;
        }
    }
}
```