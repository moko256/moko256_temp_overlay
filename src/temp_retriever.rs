use std::{process::Command, str::from_utf8};

pub fn retrieve_cpu_temp_c() -> Option<i32> {
    let cmd_result = Command::new("wmic")
    .args(
        r"/namespace:\\root\wmi PATH MSAcpi_ThermalZoneTemperature get CurrentTemperature /value".split(' ')
    )
    .output()
    .ok()?
    .stdout;

    let cmd_result = from_utf8(&cmd_result)
        .unwrap()
        .trim()
        .lines()
        .next()?
        .trim()
        .split('=')
        .last()?;

    Some((cmd_result.parse::<f32>().ok()? / 10.0 - 273.15).round() as i32)
}

pub fn retrieve_gpu_temp_c() -> Option<i32> {
    Some(
        from_utf8(
            Command::new("nvidia-smi")
                .args(["--query-gpu=temperature.gpu", "--format=csv"].as_ref())
                .output()
                .ok()?
                .stdout
                .as_ref(),
        )
        .ok()?
        .lines()
        .skip(1)
        .next()?
        .trim()
        .parse::<i32>()
        .ok()?,
    )
}
