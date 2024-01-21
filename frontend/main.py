from pathlib import Path

import os
import time
import json
import numpy as np
import matplotlib.pyplot as plt

# Setting some constants
CUR_DIR = os.path.dirname(os.path.realpath(__file__))
MAP_NAME = "de_dust2"

MAP_DATA_PATH = Path(Path(CUR_DIR) / "static" / MAP_NAME / "data.json")
RADAR_PNG_PATH = Path(Path(CUR_DIR) / "static" / MAP_NAME / "radar.png")
RADAR_PATH = Path(Path(CUR_DIR) / "res" / "radar.json")

with open(MAP_DATA_PATH, "r") as file:
    map_data = json.load(file)

while True:
    # Read the radar information
    try:
        with open(RADAR_PATH, "r") as file:
            radar_data = json.load(file)
    except Exception as e:
        time.sleep(0.001)

    # Get the canvas ready to draw
    plt.clf()
    image = plt.imread(RADAR_PNG_PATH)
    plt.imshow(image)

    for player in radar_data:
        # Get the scale of the radar and coordinates
        scale = map_data["scale"]
        world_space_x = map_data["x"]
        world_space_y = map_data["y"]

        # Get the player's position
        x = player["position"]["x"]
        y = player["position"]["y"]
        z = player["position"]["z"]

        # Convert the player's position to the radar's coordinate system
        player_map_x_position = (world_space_x - x) * -1.0 / scale
        player_map_y_position = (world_space_y - y) / scale

        # Determine dot color
        if player["team"] == 2:
            dot_color = 'red'
        else:
            dot_color = 'blue'

        plt.plot(player_map_x_position, player_map_y_position, marker='o', color=dot_color)

    # Show the plot
    plt.show()
