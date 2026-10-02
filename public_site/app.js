document.addEventListener('DOMContentLoaded', () => {
  const primaryBtn = document.getElementById('primary-download-btn');
  const labelEl = document.getElementById('primary-btn-label');
  const subEl = document.getElementById('primary-btn-sub');

  const releaseBaseUrl = "https://github.com/BatxRinth/BatxRinth/releases/latest/download";
  const userAgent = navigator.userAgent.toLowerCase();

  let downloadUrl = `${releaseBaseUrl}/BatxRinth_x64-setup.exe`;
  let labelText = "Download for Windows";
  let subText = "BatxRinth_x64-setup.exe (64-bit)";

  if (userAgent.includes('mac') || userAgent.includes('os x')) {
    downloadUrl = `${releaseBaseUrl}/BatxRinth_universal.dmg`;
    labelText = "Download for macOS";
    subText = "BatxRinth_universal.dmg (Apple Silicon & Intel)";
  } else if (userAgent.includes('linux')) {
    downloadUrl = `${releaseBaseUrl}/BatxRinth_amd64.AppImage`;
    labelText = "Download for Linux";
    subText = "BatxRinth_amd64.AppImage (.deb and .rpm on the releases page)";
  }

  if (primaryBtn && labelEl && subEl) {
    primaryBtn.href = downloadUrl;
    labelEl.textContent = labelText;
    subEl.textContent = subText;
  }

  // Dynamically fetch latest version tag from GitHub API
  fetch("https://api.github.com/repos/BatxRinth/BatxRinth/releases/latest")
    .then(res => res.json())
    .then(data => {
      if (data && data.tag_name) {
        const versionTagEl = document.getElementById('hero-version-tag');
        if (versionTagEl) {
          const cleanVersion = data.tag_name.replace(/^v/, '');
          versionTagEl.textContent = `Version ${cleanVersion} Released`;
        }
      }
    })
    .catch(() => {
      // Fallback kept at Version 1.0.0 Released
    });
});
