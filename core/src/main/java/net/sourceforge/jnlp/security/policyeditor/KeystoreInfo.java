/*Copyright (C) 2014 Red Hat, Inc.

This file is part of IcedTea.

IcedTea is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 2.

IcedTea is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with IcedTea; see the file COPYING.  If not, write to
the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
02110-1301 USA.

Linking this library statically or dynamically with other modules is
making a combined work based on this library.  Thus, the terms and
conditions of the GNU General Public License cover the whole
combination.

As a special exception, the copyright holders of this library give you
permission to link this library with independent modules to produce an
executable, regardless of the license terms of these independent
modules, and to copy and distribute the resulting executable under
terms of your choice, provided that you also meet, for each linked
independent module, the terms and conditions of the license of that
module.  An independent module is a module which is not derived from
or based on this library.  If you modify this library, you may extend
this exception to your version of the library, but you are not
obligated to do so.  If you do not wish to do so, delete this
exception statement from your version.
 */

package net.sourceforge.jnlp.security.policyeditor;

import java.util.Objects;

// http://docs.oracle.com/javase/7/docs/technotes/guides/security/PolicyFiles.html
public class KeystoreInfo {

    private final String keyStoreUrl;
    private final String keyStoreType;
    private final String keyStoreProvider;
    private final String keyStorePasswordUrl;

    public KeystoreInfo(final String keyStoreUrl, final String keyStoreType, final String keyStoreProvider, final String keyStorePasswordUrl) {
        this.keyStoreUrl = keyStoreUrl;
        this.keyStoreType = keyStoreType;
        this.keyStoreProvider = keyStoreProvider;
        this.keyStorePasswordUrl = keyStorePasswordUrl;
    }

    public String getKeyStoreUrl() {
        return keyStoreUrl;
    }

    public String getKeyStoreType() {
        return keyStoreType;
    }

    public String getKeyStoreProvider() {
        return keyStoreProvider;
    }

    public String getKeyStorePasswordUrl() {
        return keyStorePasswordUrl;
    }

    @Override
    public String toString() {
        return "KeystoreInfo{" +
            "keyStoreUrl='" + keyStoreUrl + '\'' +
            ", keyStoreType='" + keyStoreType + '\'' +
            ", keyStoreProvider='" + keyStoreProvider + '\'' +
            ", keyStorePasswordUrl='" + keyStorePasswordUrl + '\'' +
        '}';
    }

    @Override
    public boolean equals(final Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;

        final KeystoreInfo that = (KeystoreInfo) o;

        if (!keyStorePasswordUrl.equals(that.keyStorePasswordUrl)) {
            return false;
        }
        if (!keyStoreProvider.equals(that.keyStoreProvider)) {
            return false;
        }
        if (!keyStoreType.equals(that.keyStoreType)) {
            return false;
        }
        if (!keyStoreUrl.equals(that.keyStoreUrl)) {
            return false;
        }

        return true;
    }

    @Override
    public int hashCode() {
        return Objects.hash(keyStoreUrl,  keyStoreType, keyStoreProvider, keyStorePasswordUrl);
    }
}
