// Basic WebSocket Connection

let socket = new WebSocket("ws://127.0.0.1:3012");

socket.onopen = function(e) {
    console.log('Battle cruiser operational');
    socket.send("Battle cruiser operational");
    console.log(e)
};

let id = val => document.getElementById(val),
    start = id('start-recording'),
    stop = id('stop-recording'),
    player_id,
    stream,
    recorder,
    recordedChunks = [],
    chunkTimesliceMillis = 500;

start.onclick = function() {
    start.disabled = true;
    stop.removeAttribute('disabled');

    navigator.mediaDevices.getUserMedia({
        audio: true,
        video: false
    })
    .then(function(_stream) {
        // Docs: https://developer.mozilla.org/en-US/docs/Web/API/MediaStream
        stream = _stream;
        console.log(stream.getAudioTracks());

        let player_id = 6;

        // Docs: https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder
        recorder = new MediaRecorder(stream);
        recorder.ondataavailable = event => {
            // event.data type is `Blob`
            // Docs: https://developer.mozilla.org/en-US/docs/Web/API/Blob
            if (!player_id) {
                console.log("No player ID received from backend, not sending audio chunks");
            } else {
                // Create new blob with first byte indicating player ID
                let blob_with_player_id = new Blob([6 + event.data]);
                socket.send(blob_with_player_id);
            }
        };
        recorder.start(chunkTimesliceMillis);
    })
    .catch(function(err) {
        console.error(err);
        alert("Error happened with MediaStream or MediaRecorder, check console for details.");
    });
};

stop.onclick = function() {
    start.removeAttribute('disabled');
    stop.disabled = true;
    recorder.stop();
};

start.disabled = false;