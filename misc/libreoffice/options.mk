# $NetBSD: options.mk,v 1.12 2020/10/24 16:51:49 tnn Exp $

PKG_OPTIONS_VAR=	PKG_OPTIONS.libreoffice
PKG_SUPPORTED_OPTIONS=	java debug gtk3 cups ldap dbus
PKG_SUGGESTED_OPTIONS=	ldap dbus

.include "../../mk/bsd.prefs.mk"

# "checking the installed JDK... configure: error: JDK is too old, you need at least 9"
#
# Only enable Java on platforms where OpenJDK>=9 is the default,
# see mk/java-vm.mk
.if !empty(MACHINE_PLATFORM:MNetBSD-[789].*-i386) || \
    !empty(MACHINE_PLATFORM:MNetBSD-[789].*-x86_64)
PKG_SUGGESTED_OPTIONS+=	java
.endif

.include "../../mk/bsd.options.mk"

PLIST_VARS+=	java gtk3 ldap

.if !empty(PKG_OPTIONS:Mjava)
.include "../../mk/java-env.mk"
.include "../../mk/java-vm.mk"
USE_JAVA=		yes
USE_JAVA2=		yes
BUILD_DEPENDS+=	apache-ant-[0-9]*:../../devel/apache-ant
CONFIGURE_ARGS+=	--with-ant-home=${PREFIX}

DEPENDS+=		hsqldb18-[0-9]*:../../databases/hsqldb18
CONFIGURE_ARGS+=	--with-hsqldb-jar=${PREFIX}/lib/java/hsqldb18/hsqldb.jar
CONFIGURE_ARGS+=	--enable-ext-wiki-publisher \
			--with-java \
			--with-jdk-home=${PKG_JAVA_HOME} \
			--without-system-beanshell \
			--enable-scripting-beanshell \
			--with-system-hsqldb \
			--without-system-jfreereport
PLIST_SRC=		${PLIST_SRC_DFLT:Q} ${PKGDIR}/PLIST.java
PLIST.java=		yes
.else
CONFIGURE_ARGS+=	--without-java
.endif

.if !empty(PKG_OPTIONS:Mdebug)
CONFIGURE_ARGS+=	--enable-debug
PLIST_SRC=		${PLIST_SRC_DFLT:Q} ${PKGDIR}/PLIST.debug
.else
CONFIGURE_ARGS+=	--enable-release-build
.endif

.if !empty(PKG_OPTIONS:Mgtk3)
CONFIGURE_ARGS+=	--enable-gtk3
PLIST.gtk3=		yes
.else
CONFIGURE_ARGS+=	--disable-gtk3
.endif

.if !empty(PKG_OPTIONS:Mcups)
CONFIGURE_ARGS+=	--enable-cups
.else
CONFIGURE_ARGS+=	--disable-cups
.endif

.if !empty(PKG_OPTIONS:Mdbus)
CONFIGURE_ARGS+=	--enable-dbus
.else
CONFIGURE_ARGS+=	--disable-dbus
.endif

.if !empty(PKG_OPTIONS:Mldap)
CONFIGURE_ARGS+=	--enable-ldap
PLIST.ldap=		yes
.else
CONFIGURE_ARGS+=	--disable-ldap
.endif
