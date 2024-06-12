from ultralytics import YOLO # type: ignore

# Load a model
model = YOLO(r"C:\Users\29115\yolov8\ultralytics\runs\detect\train2\weights\best.pt")  # load a custom model

# Validate the model
if __name__ == '__main__':
    metrics = model.val()  # no arguments needed, dataset and settings remembered
    metrics.box.map  # map50-95
    metrics.box.map50  # map50
    metrics.box.map75  # map75
    metrics.box.maps  # a list contains map50-95 of each category