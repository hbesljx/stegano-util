# stegano-util
## 一个隐写文件到图片的rust crate

## 目前支持以下隐写方式：
- `luma_lsb_hide`：基于YCbCr中的Luma向量的lsb隐写

示例：
```
use stegano_util::{luma_lsb::luma_lsb_read, luma_lsb_hide};

#[test]
fn test_luma_lsb_hide(){
    let img_path:&str="./tests/input.jpg";
    let bin_path:&str="./tests/test.bin";

    let res=luma_lsb_hide(img_path, bin_path);
    match res {
        Ok(_)=>{println!("隐写成功!");},
        Err(err)=>{println!("{}",&err);}
    }
    
}
#[test]
fn test_luma_lsb_read(){
    let output_path="./output.png";
    let res3=luma_lsb_read(output_path);
    let mut text=Vec::new();
    match res3 {
        Ok(value)=>{text=value;},
        Err(err)=>{
            println!("{}",err);
            return;
        }
    }
    println!("{}",text.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}
```