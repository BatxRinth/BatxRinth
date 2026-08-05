document.addEventListener('DOMContentLoaded', () => {
  const primaryBtn = document.getElementById('primary-download-btn');
  const labelEl = document.getElementById('primary-btn-label');
  const subEl = document.getElementById('primary-btn-sub');

  const releaseBaseUrl = "https://github.com/BatxRinth/BatxRinth/releases/latest/download";
  const latestReleaseUrl = "https://github.com/BatxRinth/BatxRinth/releases/latest";
  const userAgent = navigator.userAgent.toLowerCase();

  let downloadUrl = latestReleaseUrl;
  let labelText = "Download for Windows";
  let subText = "BatxRinth_x64-setup.exe (64-bit)";

  if (userAgent.includes('mac') || userAgent.includes('os x')) {
    downloadUrl = latestReleaseUrl;
    labelText = "Download for macOS";
    subText = "BatxRinth.dmg (Apple Silicon & Intel)";
  } else if (userAgent.includes('linux')) {
    downloadUrl = latestReleaseUrl;
    labelText = "Download for Linux";
    subText = "BatxRinth.AppImage / .deb";
  }

  if (primaryBtn && labelEl && subEl) {
    primaryBtn.href = downloadUrl;
    labelEl.textContent = labelText;
    subEl.textContent = subText;
  }
});
