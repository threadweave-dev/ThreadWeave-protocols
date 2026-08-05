$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

& buf generate
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
