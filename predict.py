from ultralytics import YOLO

# Load a model
model = YOLO(r"C:\Users\29115\yolov8\ultralytics\runs\detect\train2\weights\best.pt")  # load an official model

# Predict with the model
if __name__ == '__main__':
    results = model("img.png")  # predict on an image