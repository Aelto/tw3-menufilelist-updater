#[test]
/// Confirm the listing of files includes all XML files, even those with the `~`
/// prefix in their name:
fn test_unfiltered_list() -> std::io::Result<()> {
  use crate::filelist::FileList;

  let mut filelist = FileList::from_directory(&"./bin/config/r4game/user_config_matrix/pc".into())?;

  let files: &mut Vec<String> = filelist.as_mut();
  files.sort();

  let mut expected = vec![
    "~ignore_test.xml",
    "audio.xml",
    "display.xml",
    "gameplay.xml",
    "gamma.xml",
    "graphics.xml",
    "graphicsdx11.xml",
    "hidden.xml",
    "hud.xml",
    "input.xml",
    "localization.xml",
    "test.xml",
  ]
  .into_iter()
  .map(str::to_owned)
  .collect::<Vec<String>>();
  expected.sort();

  assert_eq!(files, &expected);

  Ok(())
}

#[test]
/// Confirm the listing of files includes only the XML files that fit the DX11
/// filelist. Also test the DX11 list and the DX12 are different.
fn test_dx11_list() -> std::io::Result<()> {
  use crate::filelist::FileList;

  let filelist = FileList::from_directory(&"./bin/config/r4game/user_config_matrix/pc".into())?;

  // keeping the items commented to know exactly what should be filtered out:
  let expected = FileList::new(vec![
    // "~ignore_test.xml",
    "audio.xml".to_owned(),
    "display.xml".to_owned(),
    "gameplay.xml".to_owned(),
    "gamma.xml".to_owned(),
    // "graphics.xml".to_owned(),
    "graphicsdx11.xml".to_owned(),
    "hidden.xml".to_owned(),
    "hud.xml".to_owned(),
    "input.xml".to_owned(),
    "localization.xml".to_owned(),
    "test.xml".to_owned(),
  ]);

  assert_eq!(
    expected.to_dx11_only_filelist(),
    filelist.to_dx11_only_filelist()
  );

  assert_ne!(
    expected.to_dx11_only_filelist(),
    filelist.to_dx12_only_filelist()
  );

  assert_ne!(
    expected.to_dx11_only_filelist(),
    expected.to_dx12_only_filelist()
  );

  Ok(())
}

#[test]
/// Confirm the listing of files includes only the XML files that fit the DX12
/// list. Also test the DX11 list and the DX12 are different.
fn test_dx12_list() -> std::io::Result<()> {
  use crate::filelist::FileList;

  let filelist = FileList::from_directory(&"./bin/config/r4game/user_config_matrix/pc".into())?;

  // keeping the items commented to know exactly what should be filtered out:
  let expected = FileList::new(vec![
    // "~ignore_test.xml",
    "audio.xml".to_owned(),
    "display.xml".to_owned(),
    "gameplay.xml".to_owned(),
    "gamma.xml".to_owned(),
    "graphics.xml".to_owned(),
    // "graphicsdx11.xml".to_owned(),
    "hidden.xml".to_owned(),
    "hud.xml".to_owned(),
    "input.xml".to_owned(),
    "localization.xml".to_owned(),
    "test.xml".to_owned(),
  ]);

  assert_eq!(
    expected.to_dx12_only_filelist(),
    filelist.to_dx12_only_filelist()
  );

  assert_ne!(
    expected.to_dx12_only_filelist(),
    filelist.to_dx11_only_filelist()
  );

  assert_ne!(
    expected.to_dx11_only_filelist(),
    expected.to_dx12_only_filelist()
  );

  Ok(())
}

extern crate test;
