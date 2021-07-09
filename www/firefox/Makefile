# $NetBSD: Makefile,v 1.484 2021/07/02 10:51:16 tnn Exp $

FIREFOX_VER=		${MOZ_BRANCH}${MOZ_BRANCH_MINOR}
MOZ_BRANCH=		89.0
MOZ_BRANCH_MINOR=	.2

DISTNAME=	firefox-${FIREFOX_VER}.source
PKGNAME=	${DISTNAME:S/.source//:S/b/beta/:S/esr//}
CATEGORIES=	www
MASTER_SITES+=	${MASTER_SITE_MOZILLA:=firefox/releases/${FIREFOX_VER}/source/}
EXTRACT_SUFX=	.tar.xz

DISTFILES=	${DEFAULT_DISTFILES} nodejs-output-89.0.2.tgz
SITES.nodejs-output-89.0.2.tgz=	${MASTER_SITE_LOCAL}

MAINTAINER=	ryoon@NetBSD.org
HOMEPAGE=	https://www.mozilla.org/en-US/firefox/
COMMENT=	Web browser with support for extensions (version ${FIREFOX_VER:tu:C/\\.[[:digit:]\.]*//})
LICENSE=	mpl-1.1

# -------- BEFORE UPDATING THIS PACKAGE PLEASE READ & UNDERSTAND: -------
#
# This package works around a (stupid) build time dependency on nodejs
# (which is not available for all architectures and unnecessary for the
# real build). To do this, it places some additional burden on the
# maintainer.
#
# While working on the package, please make sure you have
#
#	FIREFOX_MAINTAINER=yes
#
# set in your build environment.
# When the package is ready for commit (but before commit), do:
#
#	make maintainer-files
#
# This will do another round (depending on state of your work dir at this
# moment) of one or two builds and generate a cache of all output that
# nodejs generates during a build.
#
# When this is done, just commit the results (they will be in the files/
# directory).
#
# -----------------------------------------------------------------------

UNLIMIT_RESOURCES+=	datasize virtualsize

WRKSRC=		${WRKDIR}/firefox-${FIREFOX_VER:C/b.*//}

MOZILLA_DIR=	# empty

# Note: when updating remember to conditionalise about-background.png in PLIST
CONFIGURE_ARGS+=	--enable-application=browser

#CFLAGS+=	-I${PREFIX}/include/nspr
# for lang/gcc6
CFLAGS+=	-D_GLIBCXX_INCLUDE_NEXT_C_HEADERS
CFLAGS.SunOS+=	-D_POSIX_PTHREAD_SEMANTICS

LDFLAGS+=		${COMPILER_RPATH_FLAG}${PREFIX}/lib/${PKGBASE}
LDFLAGS+=		${COMPILER_RPATH_FLAG}${PREFIX}/lib
LDFLAGS.DragonFly+=	-lplc4 -lnspr4
LDFLAGS.FreeBSD+=	-lplc4 -lnspr4
LDFLAGS.Linux+=		-lnspr4
LDFLAGS.SunOS+=		-lm

# XXX not sure how to test this! likely unnecessary
NOT_PAX_MPROTECT_SAFE+=	lib/${PKGBASE}/plugin-container

# Avoid ld "invalid section index" errors.
BUILDLINK_TRANSFORM.SunOS+=	rm:-fdata-sections
BUILDLINK_TRANSFORM.SunOS+=	rm:-ffunction-sections
BUILDLINK_TRANSFORM.SunOS+=	rm:-pie

# Workaround for https://bugs.llvm.org/show_bug.cgi?id=46366
BUILDLINK_TRANSFORM.NetBSD+=	rm:-fexperimental-new-pass-manager

SUBST_CLASSES+=			dfly_malloc_h
SUBST_STAGE.dfly_malloc_h=	pre-configure
SUBST_MESSAGE.dfly_malloc_h=	Dont include malloc.h on dragonflybsd
SUBST_SED.dfly_malloc_h=	-e 's,HAVE_MALLOC_H,HAVE_MALLOC_H \&\& !defined(__DragonFly__),g'
SUBST_FILES.dfly_malloc_h+=	media/ffvpx/libavutil/mem.c

.include "mozilla-common.mk"
.include "options.mk"

CHECK_INTERPRETER_SKIP+=	lib/firefox-sdk/sdk/bin/header.py
CHECK_INTERPRETER_SKIP+=	lib/firefox-sdk/sdk/bin/typelib.py
CHECK_INTERPRETER_SKIP+=	lib/firefox-sdk/sdk/bin/xpidl.py
CHECK_INTERPRETER_SKIP+=	lib/firefox-sdk/sdk/bin/xpt.py

CHECK_WRKREF_SKIP+=	lib/${MOZILLA}/chrome/toolkit/content/global/buildconfig.html

MOZILLA=	${PKGBASE}
MOZILLA_ICON=	${WRKSRC}/${OBJDIR}/dist/${MOZILLA}/browser/chrome/icons/default/default48.png
.if !empty(PKG_OPTIONS:Mofficial-mozilla-branding)
MOZILLA_NAME=	Firefox
.else
MOZILLA_NAME=	Browser
.endif

#
# Generate list of subst entries for various .cargo-checksum.json files.  These
# are all handled together in one big substitution to simplify things rather
# than one substitution entry per file, but are kept separate below to ease
# updating and verification.
#

CKSUM_CRATES+=	third_party/rust/libc

# patch-third__party_rust_libc_src_unix_linux__like_linux_mod.rs
CKSUMS+=	3dc8b1980062117edd7bdfdf2279065737f8cdcdcb883f57f23f27a1d0bbb8b2
CKSUMS+=	8895b1ebe70ee03b5adc5b8005e7d2d71b09d69b9964f473b885cf312c45e56b

# patch-third__party_rust_libc_src_unix_linux__like_linux_gnu_mod.rs
CKSUMS+=	c7924db757cf898c78029ad3772e6168ec99dd0dd1534df4132f8fe9717afed7
CKSUMS+=	262a461a268c01e546f073495e23a3dad5aebc473b74904c2139ed565366d9fe

CKSUM_CRATES+=	third_party/rust/nix

# patch-third__party_rust_nix_src_sys_ptrace_linux.rs
CKSUMS+=	f09b45148004f4b28d8503c397a8d112d31046c98e68335bf4e89425d5b33f07
CKSUMS+=	550959b7d04e0e3435b0a96a2ca6e38ec8874ebf8728e16659107c1651b03bef

SUBST_CLASSES+=		cksum
SUBST_STAGE.cksum=	pre-configure
.for crate in ${CKSUM_CRATES}
SUBST_FILES.cksum+=	${crate}/.cargo-checksum.json
.endfor
.for from to in ${CKSUMS}
SUBST_SED.cksum+=	-e 's,${from},${to},g'
.endfor

pre-configure:
	cd ${WRKSRC} && autoconf
	cd ${WRKSRC}/js/src && autoconf
	cd ${WRKSRC} && mkdir ${OBJDIR}
	cd ${WRKSRC}/${OBJDIR} && touch old-configure.vars
#	Do not fetch Rust Cargo file via network during build
.if !defined(FIREFOX_MAINTAINER)
	mv ${WRKDIR}/dist ${WRKSRC}/${OBJDIR}
.endif

.if defined(FIREFOX_MAINTAINER)
# Create files needed only by the firefox maintainer when updating
# the package
# XXX - manually removing the .*_done files is wrong!
.PHONY: build-list
build-list:
	cd ${WRKSRC}/${OBJDIR}/dist/bin/browser/chrome && \
		find . -type f | sort > ${OUT:Q}

NODE_LIST=	"${WRKDIR}/node.list"
NO_NODE_LIST=	"${WRKDIR}/no-node.list"
NODE_FILES=	"${WRKDIR}/node.flist"

.PHONY: maintainer-files
maintainer-files:
	rm -f ${FILESDIR}/node-wrapper.sh
	V=$$( node -v ) && \
	    printf '#! /bin/sh\n\nVERS=%s\n\nif [ "$$1" = "-v" ] || [ "$$1" = "--version" ]; then\n\tprintf "$${VERS}\\n"\nfi\n\nexit 0\n' $$V \
	    > ${FILESDIR}/node-wrapper.sh && \
	    chmod 0755 ${FILESDIR}/node-wrapper.sh
	rm -f ${WRKDIR}/.build_done ${WRKDIR}/.configure_done
	${MAKE} MAINTAINER_INTERNAL=yes build
	${MAKE} MAINTAINER_INTERNAL=yes OUT="${NO_NODE_LIST}" build-list
	${MAKE} OUT="${NODE_LIST}" build-list
	${DIFF} -u "${NO_NODE_LIST}" "${NODE_LIST}" | \
	    ${AWK} \
	    '/^\+\.\//{ printf("dist/bin/browser/chrome/%s\n", gensub(/^\+\.\//, "", "")) }' \
	    > "${NODE_FILES}"
	cd ${WRKSRC}/${OBJDIR} && tar -c -T "${NODE_FILES}" -z \
	    -f ${FILESDIR}/nodejs-output-${PKGVERSION_NOREV}.tgz
.endif

post-build:
	${SED} -e 's|@MOZILLA@|${MOZILLA}|g'				\
	  -e 's|@MOZILLA_NAME@|${MOZILLA_NAME}|g'			\
	  -e 's|@FIREFOX_ICON@|${MOZILLA}.png|g'			\
	  < ${FILESDIR}/desktop.in					\
	  > ${WRKDIR}/desktop

INSTALLATION_DIRS+=	share/applications share/pixmaps
post-install:
	${ECHO} '#! /bin/sh' > ${DESTDIR}${PREFIX}/bin/${MOZILLA}
	${ECHO} '${PREFIX}/lib/${MOZILLA}/${MOZILLA} "$$@"' >> \
		${DESTDIR}${PREFIX}/bin/${MOZILLA}
	${CHMOD} 755 ${DESTDIR}${PREFIX}/bin/${MOZILLA}
	${INSTALL_DATA} ${WRKDIR}/desktop				\
	  ${DESTDIR}${PREFIX}/share/applications/${MOZILLA}.desktop
	${INSTALL_DATA} ${MOZILLA_ICON}					\
	  ${DESTDIR}${PREFIX}/share/pixmaps/${MOZILLA}.png

.include "../../sysutils/desktop-file-utils/desktopdb.mk"
.include "../../mk/bsd.pkg.mk"
