[System.Reflection.Assembly]::LoadWithPartialName("System.windows.forms") | out-null

$OpenFileDialog = New-Object System.Windows.Forms.OpenFileDialog
$OpenFileDialog.initialDirectory = "%APPDATA%/EldenRing"
$OpenFileDialog.filter = "Elden Ring Save (*.sl2)|*.sl2|All files (*.*)|*.*"
$OpenFileDialog.ShowDialog()
$OpenFileDialog.filename
