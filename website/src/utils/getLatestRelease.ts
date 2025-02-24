export const getLatestRelease = async () => {
    let repo_name = "rustupolis";
    let org_name = "deadelineurz";
    let release_platform = {mac: "", win: "", linux: ""}
    const res = await fetch(`https://api.github.com/repos/${org_name}/${repo_name}/releases`)

    const json_releases : any[] = await res.json()

    if (json_releases.length == 0){
        return null;
    }
    const release = json_releases[0]
    if (release) {
        for (let index in release.assets) {
            let asset = release.assets[index];
            if (asset.name.includes("linux")) {
                release_platform.linux = asset.browser_download_url
            } else if (asset.name.includes("macos")) {
                release_platform.mac = asset.browser_download_url
            } else if (asset.name.includes("windows")) {
                release_platform.win = asset.browser_download_url
            }
        }
    }
    return release_platform;
}

export const getCurrentOs = (b_window : any) => {
    const platform: string = b_window?.navigator?.platform || '';
    const userAgent: string = b_window?.navigator?.userAgent || '';

    if (!platform && !userAgent) {
        return "NoOs";
    }

    const platformUpper = platform.toUpperCase();
    const userAgentUpper = userAgent.toUpperCase();

    // Android detection
    if (userAgentUpper.includes("ANDROID")) {
        return "Android";
    }
    // MacOS detection
    if (platformUpper === "MACINTEL" || platformUpper === "DARWIN") {
        return "MacOS";
    }
    // Windows detection
    if (platformUpper === "WIN32") {
        return "Windows";
    }
    // Linux detection
    if (platformUpper.includes("LINUX")  || platformUpper === "X11") {
        return "Linux";
    }
}
