# Usage: scripts\dbdroptest.ps1

# Drop all databases with _laguna_test_db in name.
# WARNING: this does not work with non-tty terminals
$env:PGPASSWORD='postgres'
(psql -U postgres -q -c '\l') | Select-String -AllMatches $args[0] | ForEach-Object {
    $db = $_.Line -split '\s*\|\s*' | Select-Object -Index 0
    # Consume the leading space
    $db_correct = $db.Substring(1)
    # FIXME: This is bit hacky and powershell sucks with escaping, but it works.
    $db_formatted = "DROP DATABASE `"`"$db_correct`"`""
    psql -U postgres -q -c "$db_formatted"
    Write-Host "DROPPED DATABASE $db_correct"
}