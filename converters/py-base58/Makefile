# $NetBSD: Makefile,v 1.1 2021/02/10 10:41:36 adam Exp $

DISTNAME=	base58-2.1.0
PKGNAME=	${PYPKGPREFIX}-${DISTNAME}
CATEGORIES=	converters python
MASTER_SITES+=	${MASTER_SITE_PYPI:=b/base58/}

MAINTAINER=	pkgsrc-users@NetBSD.org
HOMEPAGE=	https://github.com/keis/base58
COMMENT=	Base58 and Base58Check implementation
LICENSE=	mit

TEST_DEPENDS+=	${PYPKGPREFIX}-hamcrest-[0-9]*:../../devel/py-hamcrest
TEST_DEPENDS+=	${PYPKGPREFIX}-test-[0-9]*:../../devel/py-test
TEST_DEPENDS+=	${PYPKGPREFIX}-test-benchmark-[0-9]*:../../devel/py-test-benchmark

USE_LANGUAGES=	# none

PYTHON_VERSIONS_INCOMPATIBLE=	27

post-install:
	cd ${DESTDIR}${PREFIX}/bin && \
	${MV} base58 base58-${PYVERSSUFFIX} || ${TRUE}

do-test:
	cd ${WRKSRC} && ${SETENV} ${TEST_ENV} pytest-${PYVERSSUFFIX}

.include "../../lang/python/egg.mk"
.include "../../mk/bsd.pkg.mk"
