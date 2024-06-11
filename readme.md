## 安装依赖

```shell
pip install -r .\requirements.txt -i https://mirror.sjtu.edu.cn/pypi/web/simple/
```

## 数据集制作

```shell
# 整理图片
python .\extract_images.py
# 标注数据
labelme
# 删除多余的图片
python datasets_utils/remove_uesless_img.py
# 将labelemeb 标注好的数据转换成yolov8支持的格式
python datasets_utils/labelme2yolov8.py datasets/source datasets 0.8
```

## 训练

## 验证

## 预测
