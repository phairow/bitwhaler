(function() {

    const AudioContext = window.AudioContext || window.webkitAudioContext;

    const audioContext = new AudioContext();

    // get the audio element
    const audioElement = document.querySelector('#whalesound');

    // pass it into the audio context
    const track = audioContext.createMediaElementSource(audioElement);

    track.connect(audioContext.destination);

    let pubnub = new PubNub({
        publishKey: "pub-c-10921688-79ed-4759-b6e2-4388eed57ffe",
        subscribeKey: "sub-c-bc7c86ac-8ff9-11ea-9dd4-caf89c7998a9",
        uuid: "whaler_client"
    });

    const startButton = document.querySelector('#startbutton');
    startButton.addEventListener('click', function() {
        // check if context is in suspended state (autoplay policy)
        if (audioContext.state === 'suspended') {
            audioContext.resume();
        }
        pubnub.addListener({
            message: function(m) {
                audioElement.play();
                const images = document.querySelector('.images');
                images.className = "images";
                const whale = document.querySelector('#whale');
                whale.className = "";

                setTimeout(() => {
                    audioElement.pause();
                    const whale = document.querySelector('#whale');
                    whale.className = "hidden";
                }, 6000);
            },
            status: function(s) {
    
                if (s.category === "PNConnectedCategory") {
                    setTimeout(() => {
                        pubnub.publish( {
                            message: 'hello',
                            channel: 'whaler_process'
                        });    
                    }, 3000);
                }
            }
        });
        
        pubnub.subscribe({
            channels: ['whaler_process'],
        });

        startButton.className = "hidden"
    }, false);

})();