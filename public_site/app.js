document.addEventListener('DOMContentLoaded', () => {
  const primaryBtn = document.getElementById('primary-download-btn');
  const labelEl = document.getElementById('primary-btn-label');
  const subEl = document.getElementById('primary-btn-sub');

  const latestReleaseUrl = "https://github.com/BatxRinth/BatxRinth/releases";
  const userAgent = navigator.userAgent.toLowerCase();

  let downloadUrl = latestReleaseUrl;
  let labelText = "Download for Windows";
  let subText = "View Latest Releases (.exe, .dmg, .AppImage)";

  if (userAgent.includes('mac') || userAgent.includes('os x')) {
    downloadUrl = latestReleaseUrl;
    labelText = "Download for macOS";
    subText = "View Latest Releases (.dmg)";
  } else if (userAgent.includes('linux')) {
    downloadUrl = latestReleaseUrl;
    labelText = "Download for Linux";
    subText = "View Latest Releases (.AppImage / .deb)";
  }

  if (primaryBtn && labelEl && subEl) {
    primaryBtn.href = downloadUrl;
    labelEl.textContent = labelText;
    subEl.textContent = subText;
  }
});
