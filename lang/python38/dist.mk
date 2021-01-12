# $NetBSD: dist.mk,v 1.8 2021/01/02 09:51:48 adam Exp $

PY_DISTVERSION=	3.8.7
DISTNAME=	Python-${PY_DISTVERSION}
EXTRACT_SUFX=	.tar.xz
DISTINFO_FILE=	${.CURDIR}/../../lang/python38/distinfo
PATCHDIR=	${.CURDIR}/../../lang/python38/patches
MASTER_SITES=	http://www.python.org/ftp/python/${PY_DISTVERSION}/
