import os
from xls_extract import extract_images_from_xls, extract_images_from_xlsx


def recursive_traversal_and_extract_images(root_dir, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    count = 0
    for subdir, _, files in os.walk(root_dir):
        for file in files:
            file_path = os.path.join(subdir, file)
            file_path = os.path.abspath(file_path)
            if file.endswith('.xlsx'):
                extract_images_from_xlsx(file_path, output_dir)
                count += 1
            elif file.endswith('.xls'):
                extract_images_from_xls(file_path, output_dir)
                count += 1
            print(count)

if __name__ == "__main__":
    root_directory = "./xlsx"
    output_directory = "./datasets/image"
    if not os.path.exists(output_directory):
        os.makedirs(output_directory)
    recursive_traversal_and_extract_images(root_directory, output_directory)