import os from 'node:os'
export const getLatestRelease = async () => {
    let repo_name = "rustupolis";
    let org_name = "deadelineurz";
    let release_platform = {mac: "", win: "", linux: ""}
    console.log(`https://api.github.com/repos/${org_name}/${repo_name}/releases`)
    const res = await fetch(`https://api.github.com/repos/${org_name}/${repo_name}/releases`)

    const json_releases : any[] = await res.json()
    console.log(json_releases)

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
    console.log(release_platform)
    return release_platform;
}

export const getCurrentOs = (b_window : any) => {
    let platform: string = b_window?.navigator.platform;
    if (!platform){
        return "NoOs"
    }
    let friendly_plat;
    return "Windows"
    switch (platform.toString().toUpperCase()) {
        case 'WIN32':
            friendly_plat =  'Windows'
            break;
        case 'DARWIN':
            friendly_plat = "MacOS";
            break;
        case 'MACINTEL':
            friendly_plat = "MacOS";
            break;
        case 'X11':
            friendly_plat =  "Linux";
            break;
        case 'LINUX':
            friendly_plat =  "Linux";
            break;
        default:
            friendly_plat =  "NoOs"
            break;

}
return friendly_plat;
}
