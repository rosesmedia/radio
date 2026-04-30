import 'https://esm.sh/preact@10.27.2/debug';
import { h, render } from 'https://esm.sh/preact@10.27.2/';
import { useCallback, useEffect, useRef, useState } from 'https://esm.sh/preact@10.27.2/hooks';
import htm from 'https://esm.sh/htm@3.1.1/';
import Hls from 'https://esm.sh/hls.js@1.6.14/';

const HLS_MIME = 'application/vnd.apple.mpegurl';

const html = htm.bind(h);

function Player(props) {
    const audio = useRef(null);
    const [isPaused, setIsPaused] = useState(true);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (!audio.current) return;
        if (!props.url) return;
        setIsPaused(true);
        setLoading(true);
        if (Hls.isSupported()) {
            console.log('[player] using hls.js');
            const hls = new Hls({
                // we need to use the worker to avoid this until we can fix the liquidsoap script
                // https://github.com/savonet/liquidsoap/issues/4398
                // https://github.com/video-dev/hls.js/issues/7075
                enableWorker: true,
            });
            hls.on(Hls.Events.ERROR, (_, data) => {
                if (data.fatal) {
                    switch (data.type) {
                        case Hls.ErrorTypes.MEDIA_ERROR:
                            console.error('attempting to recover from fatal media error');
                            hls.recoverMediaError();
                            break;
                        case Hls.ErrorTypes.NETWORK_ERROR:
                            console.error('fatal network error occured');
                            break;
                        default:
                            console.error('unrecoverable error occurred');
                            hls.destroy();
                    }
                }
            });
            hls.loadSource(props.url);
            hls.attachMedia(audio.current);
            return () => hls.destroy();
        } if (audio.current.canPlayType(HLS_MIME)) {
            console.log('[player] using browser built-in HLS');
            audio.current.src = props.url;
        } else {
            console.error(
                'Player initialised when HLS is not supported. This should not happen!'
            );
        }
    }, [props.url]);

    const togglePause = useCallback(() => {
        if (!audio.current) return;
        if (loading) return;
        if (audio.current.paused) {
            audio.current.play();
        } else {
            audio.current.pause();
        }
    }, [audio, loading]);

    return html`
        <audio ref=${audio}
            onPlay=${() => setIsPaused(false)}
            onPause=${() => setIsPaused(true)}
            onStalled=${(e) => setLoading( // // safari iOS seems to send the onWaiting event when the stream is still playing, so we make sure we aren't been lied to
                e.currentTarget.readyState < HTMLMediaElement.HAVE_FUTURE_DATA
            )}
            onPlaying=${() => setLoading(false)}
            onCanPlay=${() => setLoading(false)}
            onLoadedMetadata=${() => setLoading(false)}
            onLoadedData=${(e) => {
                if (e.currentTarget.readyState >= HTMLMediaElement.HAVE_FUTURE_DATA) {
                    setLoading(false);
                }
            }}
        ></audio>

        ${props.url && html`<button class="btn btn-outline-primary btn-lg" type="button" disabled=${loading} onclick=${togglePause}>
            ${loading ? html`
                <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
                <span class="visually-hidden" role="status">Loading...</span>
            ` : html`
                <i class="bi ${isPaused ? 'bi-play' : 'bi-pause'}" style="font-size: 1.5rem"></i>
                <span class="visually-hidden" role="status">${isPaused ? 'Play' : 'Pause'}</span>
            `}
        </button>`}
    `;
}

if (!(new Audio().canPlayType(HLS_MIME) !== '' || Hls.isSupported())) {
    // TODO: lets not do this
    document.write('your browser does not support HLS');
    throw new Error('browser does not support HLS');
}

const root = document.getElementById('player');
const streamUrl = root.dataset.streamUrl;

render(
    html`<${Player} url=${streamUrl} />`,
    root,
)
