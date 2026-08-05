document.addEventListener('DOMContentLoaded', () => {
  const primaryBtn = document.getElementById('primary-download-btn');
  const labelEl = document.getElementById('primary-btn-label');
  const subEl = document.getElementById('primary-btn-sub');

  const releaseBaseUrl = "https://github.com/BatxRinth/BatxRinth/releases/download/v1.0.0";
  const userAgent = navigator.userAgent.toLowerCase();

  let downloadUrl = `${releaseBaseUrl}/BatxRinth_x64-setup.exe`;
  let labelText = "Download for Windows";
  let subText = "BatxRinth_x64-setup.exe (64-bit)";

  if (userAgent.includes('mac') || userAgent.includes('os x')) {
    downloadUrl = "https://github.com/BatxRinth/BatxRinth/releases/tag/v1.0.0";
    labelText = "Download for macOS";
    subText = "BatxRinth.dmg (Apple Silicon & Intel)";
  } else if (userAgent.includes('linux')) {
    downloadUrl = "https://github.com/BatxRinth/BatxRinth/releases/tag/v1.0.0";
    labelText = "Download for Linux";
    subText = "BatxRinth.AppImage / .deb";
  }

  if (primaryBtn && labelEl && subEl) {
    primaryBtn.href = downloadUrl;
    labelEl.textContent = labelText;
    subEl.textContent = subText;
  }
});
