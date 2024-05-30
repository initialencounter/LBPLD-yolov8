import os
from PIL import Image as PILImage
from shutil import copy as shutil_copy

def copy_xlsx(xlsx_path, output_dir):
    try:
        shutil_copy(xlsx_path, output_dir)
    except Exception as e:
        print(f"Error processing {xlsx_path}: {e}")

def recursive_traversal_and_extract_images(root_dir, output_dir):
    for subdir, _, files in os.walk(root_dir):
        for file in files:
            file_path = os.path.join(subdir, file)
            print(file_path)
            if file.endswith('.xlsx'):
                copy_xlsx(file_path, output_dir+"x")
            elif file.endswith('.xls'):
                copy_xlsx(file_path, output_dir)

if __name__ == "__main__":
    root_directory = "E:\\2023\\2023-07"
    output_directory = "xls"
    if not os.path.exists(output_directory):
        os.makedirs(output_directory)
    if not os.path.exists(output_directory+"x"):
        os.makedirs(output_directory+"x")
    recursive_traversal_and_extract_images(root_directory, output_directory)
