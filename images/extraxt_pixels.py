import json
import os
from PIL import Image

def channels_extract(channels_list):
    px_value = 0
    for ch in channels_list:
            px_value = px_value * 256
            px_value = px_value + int(ch)
    return px_value


def images_to_json(first_image_path, second_image_path):
    # Open the image
    first_img = Image.open(first_image_path).convert("RGBA")
    second_img = Image.open(second_image_path).convert("RGBA")

    # Convert pixel values to strings
    first_pixel_strings = [channels_extract(list(px)) for px in list(first_img.getdata())]
    second_pixel_strings = [channels_extract(list(px)) for px in list(second_img.getdata())]
        
    # Prepare the JSON object
    data = {"orig_pixels": first_pixel_strings, "new_pixels": second_pixel_strings}

    # Save JSON to a file
    json_filename = "input.json"
    with open(json_filename, 'w') as json_file:
        json.dump(data, json_file, indent=4)

    print(f"JSON file '{json_filename}' generated successfully!")

# Example usage
if __name__ == "__main__":
    images_to_json("cropped_pepa.png", "new_pepa.png")