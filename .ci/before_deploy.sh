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
    cp target/$TARGET/release/libsystem_uri.so $stage/ || true
    # and mac
    cp target/$TARGET/release/libsystem_uri.dylib $stage/ || true
    cp README.md $stage/
    cp LICENSE $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
