# source this file to your environment
# source .metaloot/bin/startloot

stoploot () {
    if [ -n "${BASH:-}" -o -n "${ZSH_VERSION:-}" ] ; then
        hash -r 2> /dev/null
    fi

    if [ -n "${_OLD_VIRTUAL_PS1:-}" ] ; then
        PS1="${_OLD_VIRTUAL_PS1:-}"
        export PS1
        unset _OLD_VIRTUAL_PS1
    fi

    unset LOOT_ENV
    unset LOOT_ROOT

    if [ ! "${1:-}" = "nondestructive" ] ; then
        unset -f stoploot
    fi
}

stoploot nondestructive

LOOT_ENV="/work/security/lootbagger/target/debug/test/.metaloot/"
export LOOT_ENV
LOOT_ROOT="/work/security/lootbagger/target/debug/test/"
export LOOT_ROOT

_OLD_VIRTUAL_PATH="$PATH"
PATH="$LOOT_ENV/bin:$PATH"
export PATH

if [ -z "${LOOT_ENV_DISABLE_PROMPT:-}" ] ; then
    _OLD_VIRTUAL_PS1="${PS1:-}"
    PS1="(lootenv) ${PS1:-}"
    export PS1
fi

if [ -n "${BASH:-}" -o -n "${ZSH_VERSION:-}" ] ; then
    hash -r 2> /dev/null
fi
