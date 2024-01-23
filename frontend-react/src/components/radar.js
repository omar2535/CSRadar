
import React, { Component } from 'react';

const websocketURL = 'ws://localhost:3030/ws';

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
            players: [],
            mapData: {
                x: 0,
                y: 0,
                scale: 0
            }
        }
    }

    selectionChange = (event) => {
        this.setState({ selectedMap: event.target.value });
        this.updateMap();
    }

    updateMap = () => {
        fetch(`/static/${this.state.selectedMap}/data.json`)
            .then(response => response.json())
            .then(mapData => {
                this.setState({
                    mapData: {
                        x: mapData['x'],
                        y: mapData['y'],
                        scale: mapData['scale']
                    }
                });
            })
            .catch(error => {
                console.error('Error reading Map data.json:', error);
            })
    }

    updateRadar = () => {
        const socket = new WebSocket(websocketURL);

        socket.onmessage = (event) => {
            console.log(event);
            const playerData = JSON.parse(event.data);
            const canvas = document.getElementById('radar-canvas');
            const ctx = canvas.getContext('2d');
            const image = document.getElementById('radar-image');

            canvas.width = image.width;
            canvas.height = image.height;
            ctx.drawImage(image, 0, 0);

            playerData.forEach(player => {
                // Perform operations on each player
                let player_x = (this.state.mapData.x - player['position']['x']) * -1.0 / this.state.mapData.scale;
                let player_y = (this.state.mapData.y - player['position']['y']) * 1.0 / this.state.mapData.scale;
                let player_team = player['team'];

                // Set player color
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
                    ctx.beginPath();
                    ctx.arc(player_x, player_y, 10, 0, 2 * Math.PI);
                    ctx.fillStyle = player_color;
                    ctx.fill();
                    ctx.closePath();
                }
            });
        };

        socket.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    };

    componentDidMount() {
        // we don't need to set an interval or anything because the updateRadar() is triggered everytime an
        // update is received from the websocket
        this.updateMap();
        this.updateRadar();
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
