import json

import subprocess


def ffprobe(path: str):
    output = subprocess.check_output(['ffprobe', '-v', 'quiet', '-print_format', 'json', '-show_format', path])
    return json.loads(output.decode('utf-8'))

def loudnorm(input_path: str, output_path: str, target_lufs: float=-12, target_lra: float=4, target_true_peak: float = -1):
    """
    :param input_path: Path to the source audio file
    :param output_path: Path to the output normalised audio file. Must be a .flac file
    :param target_lufs: Target loudness in LUFS
    :param target_lra:  Target loudness range in LUs
    :param target_true_peak: Target True Peak value (in dB)
    """
    output = subprocess.check_output([
        'ffmpeg',
        '-hide_banner', '-nostats',
        '-i', input_path,
        '-af', f'loudnorm=print_format=json:i={target_lufs}',
        '-f', 'null', '-',
    ], stderr=subprocess.STDOUT).decode('utf-8')
    data = ''
    reading_loudnorm = False
    for line in output.split():
        if reading_loudnorm:
            data += line
            if '}' in line:
                reading_loudnorm = False
        elif "{" in line:
            data += line
            reading_loudnorm = True

    print(data)

    loudnorm_info = json.loads(data)

    i = loudnorm_info["input_i"]
    tp = loudnorm_info["input_tp"]
    thresh = loudnorm_info["input_thresh"]
    lra = loudnorm_info["input_lra"]
    subprocess.call([
        'ffmpeg',
        '-loglevel', 'quiet',
        '-y',
        '-i', input_path,
        '-af', f'loudnorm=measured_i={i}:measured_tp={tp}:measured_thresh={thresh}:measured_lra={lra}:i={target_lufs}:lra={target_lra}:tp={target_true_peak}',
        '-b:a', '192k',
        '-f', 'mp3',
        output_path,
    ])
