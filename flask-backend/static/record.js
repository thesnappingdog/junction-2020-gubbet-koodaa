let id = val => document.getElementById(val),
    start = id('start-recording'),
    stop = id('stop-recording'),
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

        // Docs: https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder
        recorder = new MediaRecorder(stream);
        recorder.ondataavailable = e => {
            // ToDo: Push chunk over websocket to be analyzed on backend
            console.log("got " + chunkTimesliceMillis + "ms chunk of data");
            recordedChunks.push(e.data);
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