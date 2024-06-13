from ultralytics import YOLO # type: ignore
# Load the YOLOv8 model
model = YOLO("../ultralytics/runs/detect/train4/weights/best.pt")

if __name__ == '__main__':
    # Export the model to NCNN format
    model.export(format="onnx")  #