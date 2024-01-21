from pathlib import Path

import json
import numpy
import os

# Setting some constants
MAP_NAME = "de_dust2"
RADAR_PATH = "res/radar.json"

background_path = Path(f"res/{MAP_NAME}/background.png")
map_data_path = Path(f"res/{MAP_NAME}/data.json")
radar_path = Path(f"res/{MAP_NAME}/radar.png")

with open(map_data_path, "r") as file:
    map_data = json.load(file)

with open(RADAR_PATH, "r") as file:
    radar_data = json.load(file)


for player in radar_data["players"]:
    # Get the scale of the radar and coordinates
    scale = radar_data["scale"]
    world_space_x = radar_data["x"]
    world_space_y = radar_data["y"]

    # Get the player's position
    x = player["position"]["x"]
    y = player["position"]["y"]
    z = player["position"]["z"]

    # Convert the player's position to the radar's coordinate system
    player_map_x_position = (world_space_x - x) / scale
    player_map_y_position = (world_space_y - y) / scale





# Do something with the data
breakpoint()
