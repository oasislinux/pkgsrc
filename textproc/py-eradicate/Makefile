# $NetBSD: Makefile,v 1.2 2019/01/22 10:44:10 adam Exp $

DISTNAME=	eradicate-1.0
PKGNAME=	${PYPKGPREFIX}-${DISTNAME}
CATEGORIES=	textproc python
MASTER_SITES=	${MASTER_SITE_PYPI:=e/eradicate/}

MAINTAINER=	pkgsrc-users@NetBSD.org
HOMEPAGE=	https://github.com/myint/eradicate
COMMENT=	Removes commented-out code
LICENSE=	mit

USE_LANGUAGES=	# none

post-install:
	cd ${DESTDIR}${PREFIX}/bin && \
		${MV} eradicate eradicate-${PYVERSSUFFIX} || ${TRUE}

.include "../../lang/python/distutils.mk"
.include "../../mk/bsd.pkg.mk"
