# $NetBSD: Makefile,v 1.22 2020/12/04 20:45:36 nia Exp $
#
.include "../../news/hellanzb/Makefile.version"

DISTNAME=		hellanzb-${HELLANZB_VERSION}.${HELLANZB_SVN_REV}
PKGREVISION=		6
CATEGORIES=		news
MASTER_SITES=		${MASTER_SITE_LOCAL}

MAINTAINER=		pkgsrc-users@NetBSD.org
#HOMEPAGE=		http://www.hellanzb.com/
HOMEPAGE=		https://github.com/pjenvey/hellanzb
COMMENT=		Command line nntp file grabber
LICENSE=		modified-bsd

DEPENDS+=	${PYPKGPREFIX}-twisted>=1.3:../../net/py-twisted
DEPENDS+=	${PYPKGPREFIX}-expat-[0-9]*:../../textproc/py-expat
DEPENDS+=	par2>=0.4:../../archivers/par2
DEPENDS+=	unrar>=3.5.4:../../archivers/unrar

PYTHON_VERSIONS_ACCEPTED=	27 # as of 0.14

USE_LANGUAGES=	#none

EGDIR=		${PREFIX}/share/examples/${PKGBASE}
WRKSRC=		${WRKDIR}/hellanzb-${HELLANZB_VERSION}-trunk

CONF_FILES=	${EGDIR}/hellanzb.conf ${PKG_SYSCONFDIR}/hellanzb.conf

SUBST_CLASSES+=		conf
SUBST_FILES.conf=	${WRKDIR}/hellanzb.conf
SUBST_MESSAGE.conf=	Performing pathname substitutions in hellanzb.conf.
SUBST_STAGE.conf=	pre-install
SUBST_VARS.conf=	PREFIX

post-extract:
	${CP} ${FILESDIR}/hellanzb.conf ${WRKDIR}

post-install:
	${INSTALL_DATA_DIR} ${DESTDIR}${EGDIR}
	${INSTALL_DATA} ${WRKDIR}/hellanzb.conf ${DESTDIR}${EGDIR}

EGG_NAME=	hellanzb-${HELLANZB_VERSION}_trunk

.include "../../lang/python/distutils.mk"
.include "../../mk/bsd.pkg.mk"
