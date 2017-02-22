# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --target $TARGET --release

    # copy linux
    cp target/$TARGET/release/libsystem_uri.* $stage/
    cp README.md $stage/
    cp LICENSE $stage/

    cd $stage

    if [ -z $TARGET_NAME ]; then
        zip $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET_NAME.zip *
    else
        zip $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.zip *
    fi
    cd $src



    rm -rf $stage
}

main
