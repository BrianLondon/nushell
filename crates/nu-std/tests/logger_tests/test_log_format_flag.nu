use std *
use commons.nu *

def run-command [
    system_level,
    message_level,
    message,
    --format: string,
    --short
] {
    do {
        if $short {
            ^$nu.current-exe --commands $'use std; NU_LOG_LEVEL=($system_level) std log ($message_level) --format "($format)" --short "($message)"'
        } else {
            ^$nu.current-exe --commands $'use std; NU_LOG_LEVEL=($system_level) std log ($message_level) --format "($format)" "($message)"'
        }
    } | complete | get --ignore-errors stderr
}


def "assert formatted" [
    message: string,
    format: string,
    command_level: string
    --short
] {
    let output = (run-command "debug" $command_level $message --format $format)
    let prefix = if $short {
            ($env.LOG_SHORT_PREFIX | get ($command_level | str upcase))
        } else {
            ($env.LOG_PREFIX | get ($command_level | str upcase))
        }
    let ansi = if $short {
            ($env.LOG_ANSI | get ($command_level | str upcase))
        } else {
            ($env.LOG_ANSI | get ($command_level | str upcase))
        }

    assert equal ($output | str trim --right) (format-message $message $format $prefix $ansi)
}

def "test_format_flag" [] {
    assert formatted "test" "25 %MSG% %ANSI_START% %LEVEL%%ANSI_STOP%" critical
    assert formatted "test" "25 %MSG% %ANSI_START% %LEVEL%%ANSI_STOP%" error
    assert formatted "test" "25 %MSG% %ANSI_START% %LEVEL%%ANSI_STOP%" warning
    assert formatted "test" "25 %MSG% %ANSI_START% %LEVEL%%ANSI_STOP%" info
    assert formatted "test" "25 %MSG% %ANSI_START% %LEVEL%%ANSI_STOP%" debug
    assert formatted --short "test" "TEST %ANSI_START% %MSG%%ANSI_STOP%" critical
    assert formatted --short "test" "TEST %ANSI_START% %MSG%%ANSI_STOP%" error
    assert formatted --short "test" "TEST %ANSI_START% %MSG%%ANSI_STOP%" warning
    assert formatted --short "test" "TEST %ANSI_START% %MSG%%ANSI_STOP%" info
    assert formatted --short "test" "TEST %ANSI_START% %MSG%%ANSI_STOP%" debug
}
