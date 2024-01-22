
import React, { Component } from 'react';

class Radar extends Component {
    constructor() {
        super();
        this.mapNames = [
            'de_dust2',
            'de_inferno',
            'de_mirage',
            'de_nuke',
            'de_overpass'
        ];
        this.state = {
            selectedMap: 'de_dust2',
            players: []
        }
    }

    selectionChange = (event) => {
        this.setState({ selectedMap: event.target.value });
    }

    updateRadar = () => {
        const canvas = document.getElementById('radar-canvas');
        const ctx = canvas.getContext('2d');
        const image = document.getElementById('radar-image');

        fetch('/radar.json')
        .then(response => response.json())
        .then(playerData => {
            fetch(`/static/${this.state.selectedMap}/data.json`)
                .then(response => response.json())
                .then(mapData => {
                    canvas.width = image.width;
                    canvas.height = image.height;
                    ctx.drawImage(image, 0, 0);


                    playerData.forEach(player => {
                        // Perform operations on each player
                        let player_x = (mapData['x'] - player['position']['x']) * -1.0 / mapData['scale'];
                        let player_y = (mapData['y'] - player['position']['y']) * 1.0 / mapData['scale'];
                        let player_team = player['team'];

                        // set player color
                        let r = 0;
                        let g = 255 * (100 - player["health"]) / 100;
                        let b = 0;
                        if (player_team === 2) {
                            r = 255;
                        } else {
                            b = 255;
                        }

                        let player_color = `rgb(${r}, ${g}, ${b})`

                        if (player["health"] >= 0) {
                            // console.log(player_x, player_y);
                            ctx.beginPath();
                            ctx.arc(player_x, player_y, 10, 0, 2 * Math.PI);
                            ctx.fillStyle = player_color;
                            ctx.fill();
                            ctx.closePath();
                        }
                    });
                });
        })
        .catch(error => {
            console.error('Error reading radar.json:', error);
        });
    }

    componentDidMount() {
        this.updateRadar();
        setInterval(this.updateRadar, 500);
    }

    render() {
        const { selectedMap } = this.state;
        const imagePath = `/static/${selectedMap}/radar.png`;

        return (
            <div>
                <h1>CSRadar</h1>
                <select onChange={this.selectionChange}>
                    {this.mapNames.map((folderName, index) => (
                        <option key={index} value={folderName}>{folderName}</option>
                    ))}
                </select>
                <canvas id="radar-canvas" style={{ width: '100vw' }}>
                    <img id="radar-image" src={imagePath} alt="Radar" style={{ width: '100vw' }} />
                </canvas>
            </div>
        );
    }
}

export default Radar;
