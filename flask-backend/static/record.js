if (location.href.indexOf('file:') == 0) {
    document.write('<h1 style="color:red;">Please load this HTML file on HTTP or HTTPS.</h1>');
}

// Older browsers might not implement mediaDevices at all, so we set an empty object first
if (navigator.mediaDevices === undefined) {
    navigator.mediaDevices = {};
}

// Some browsers partially implement mediaDevices. We can't just assign an object
// with getUserMedia as it would overwrite existing properties.
// Here, we will just add the getUserMedia property if it's missing.
if (navigator.mediaDevices.getUserMedia === undefined) {
    navigator.mediaDevices.getUserMedia = function(constraints) {

        // First get ahold of the legacy getUserMedia, if present
        var getUserMedia = navigator.webkitGetUserMedia || navigator.mozGetUserMedia;

        // Some browsers just don't implement it - return a rejected promise with an error
        // to keep a consistent interface
        if (!getUserMedia) {
            return Promise.reject(new Error('getUserMedia is not implemented in this browser'));
        }

        // Otherwise, wrap the call to the old navigator.getUserMedia with a Promise
        return new Promise(function(resolve, reject) {
            getUserMedia.call(navigator, constraints, resolve, reject);
        });
    }
}

var startRecording = document.getElementById('start-recording');
var stopRecording = document.getElementById('stop-recording');

var recordAudio;
startRecording.onclick = function() {
    startRecording.disabled = true;
    navigator.mediaDevices.getUserMedia({
        audio: true,
        video: false
    }, function(stream) {
        mediaStream = stream;

        recordAudio = RecordRTC(stream, {
            type: 'audio',
            recorderType: StereoAudioRecorder,
            onAudioProcessStarted: function() {

            }
        });

        recordAudio.startRecording();

        stopRecording.disabled = false;
    }, function(error) {
        alert(JSON.stringify(error));
    });
};

stopRecording.onclick = function() {
    startRecording.disabled = false;
    stopRecording.disabled = true;

    // stop audio recorder
    recordAudio.stopRecording(function() {

        // get audio data-URL
        recordAudio.getDataURL(function(audioDataURL) {

            var audio = {
                type: recordAudio.getBlob().type || 'audio/wav',
                dataURL: audioDataURL
            };

            socketio.emit('audioRecording', audio);

            if (mediaStream) mediaStream.stop();
        });
    });
};

startRecording.disabled = false;

alert("loaded");