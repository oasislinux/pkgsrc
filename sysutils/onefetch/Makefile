# $NetBSD: Makefile,v 1.12 2021/07/05 08:23:25 pin Exp $

DISTNAME=	onefetch-2.10.2
CATEGORIES=	sysutils
MASTER_SITES=	${MASTER_SITE_GITHUB:=o2sh/}
GITHUB_TAG=	v${PKGVERSION_NOREV}

MAINTAINER=	pin@NetBSD.org
HOMEPAGE=	https://github.com/o2sh/onefetch/
COMMENT=	Git repository summary on your terminal
LICENSE=	mit

.include "cargo-depends.mk"

USE_LANGUAGES+=		c	# rust, but needs a toolchain to link
USE_TOOLS+=		pkg-config

INSTALLATION_DIRS=	bin ${PKGMANDIR}/man1

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/target/release/onefetch ${DESTDIR}${PREFIX}/bin
	${INSTALL_MAN} ${WRKSRC}/docs/onefetch.1 ${DESTDIR}${PREFIX}/${PKGMANDIR}/man1/onefetch.1

DEPENDS+=		git-base-[0-9]*:../../devel/git-base

.include "../../lang/rust/cargo.mk"
.include "../../mk/bsd.pkg.mk"
