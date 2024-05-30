import os


def remove_images(root_dir):
    for subdir, _, files in os.walk(root_dir):
        for file in files:
            file_path = os.path.join(subdir, file)
            label_path = file_path
            if not file.endswith('.png'):
                continue
            label_path = label_path.replace(".png", ".json")
            if not os.path.exists(label_path):
                os.remove(file_path)
                print(file_path)


if __name__ == "__main__":
    root_directory = "datasets/image"
    remove_images(root_directory)
