#!/usr/bin/env liquidsoap

%include "restreamer.local.conf"

settings.frame.audio.channels := 2
settings.frame.audio.samplerate := 48000

settings.server.socket := true

def get_env(key) =
  value = environment.get(key)
  if value != "" then
    value
  else
    log.critical("Must set the `#{key}` environment variable")
    shutdown(code=123)
    ""
  end
end

stream_id = get_env("STREAM_ID")
ingest_url = get_env("INGEST_URL")

settings.server.socket.path := "/tmp/restreamer-control-#{stream_id}.sock"

# def notify_ready() =
#   # the endpoint doesn't need any request body values, but liquidsoap json.stringify serialises {} as an empty array
#   ignore(http.post(
#     data = json.stringify({
#       ready = true,
#     }),
#     headers = [
#       ("Content-Type", "application/json; charset=UTF-8"),
#       ("Authorization", "Bearer #{api_token}")
#     ],
#     "#{api_base}/api/stream/#{stream_id}/ready"
#   ))
# end

ob_in = stereo(input.http(ingest_url, self_sync=false))
beeps = metronome(12.0, frequency=1000.0)

ob = fallback([ob_in, beeps], track_sensitive=false)

selected_source = interactive.int("source", 4, description="Active source (1: live input, 2: pre-stream sustainer, 3: post-stream sustainer, 4: technical difficulties sustainer)")

starting = amplify(0.5, single("#{audio_base}/starting.wav"))
ended = amplify(0.5, single("#{audio_base}/ended.wav"))
techdiff = amplify(0.5, single("#{audio_base}/techdiff.wav"))

stream = switch(
  [
    ({selected_source() == 1}, ob),
    ({selected_source() == 2}, starting),
    ({selected_source() == 3}, ended),
    ({selected_source() == 4}, techdiff),
    ({true}, techdiff),
  ],
  track_sensitive=false
)

aac_high =
  %ffmpeg(
    format = "mpegts",
    interleaved = false,
    %audio(
      codec = "aac",
      channels = 2,
      ar = 48000,
      b = "192k",
      profile = "aac_low"
    )
  )

output.srt(
  host=srt_host,
  port=srt_port,
  mode="caller",
  streamid="#{srt_streamid_prefix}/#{stream_id}",
  connection_timeout=10.0,
  aac_high,
  stream
)
