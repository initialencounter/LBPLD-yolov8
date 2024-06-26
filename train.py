from ultralytics import YOLO # type: ignore
# from ultralytics import settings

# settings.update({'datasets_dir': './datasets'})
model = YOLO('yolov8n.yaml').load('yolov8n.pt')  # build from YAML and transfer weights

if __name__ == '__main__':
    # Train the model
    results = model.train(data='./datasets/yolo.yaml', epochs=1000, imgsz=640)