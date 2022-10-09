(function() {
  const ip = document.getElementById('upload');
  const fReader = new FileReader();

  fReader.onloadend = _ => {
    // read base64 content and also remove the metadata that 
    // is prefixed to any base64 image data
    const b64Content = fReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ''
    );
    console.log('image reading completed', b64Content.slice(0, 20));
  }

  ip.addEventListener('change', e => {
    fReader.readAsDataURL(ip.files[0]);
  })
})();
