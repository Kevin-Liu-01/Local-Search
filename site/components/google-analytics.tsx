import Script from "next/script";

/** GA4 property for localsearch.kevinliu.studio. Measurement IDs are public; this one ships in page HTML. */
const GA_MEASUREMENT_ID = "G-8XK2GZEM82";
const PRODUCTION_HOST = "localsearch.kevinliu.studio";

// The site is a static export, so the production check happens in the browser:
// previews and localhost never load gtag.js. Consent Mode v2 defaults are queued
// before it loads: no ad storage anywhere, and EEA/UK/CH visitors are measured
// without analytics cookies. GA4 enhanced measurement counts client-side page views.
const GA_BOOTSTRAP = `(function(){if(location.hostname!=='${PRODUCTION_HOST}')return;
window.dataLayer=window.dataLayer||[];function gtag(){dataLayer.push(arguments);}window.gtag=gtag;
gtag('consent','default',{ad_storage:'denied',ad_user_data:'denied',ad_personalization:'denied',analytics_storage:'denied',region:['AT','BE','BG','HR','CY','CZ','DK','EE','FI','FR','DE','GR','HU','IS','IE','IT','LV','LI','LT','LU','MT','NL','NO','PL','PT','RO','SK','SI','ES','SE','CH','GB']});
gtag('consent','default',{ad_storage:'denied',ad_user_data:'denied',ad_personalization:'denied',analytics_storage:'granted'});
gtag('js',new Date());gtag('config','${GA_MEASUREMENT_ID}');
var s=document.createElement('script');s.async=true;s.src='https://www.googletagmanager.com/gtag/js?id=${GA_MEASUREMENT_ID}';document.head.appendChild(s);})();`;

export function GoogleAnalytics() {
  return (
    <Script id="ga-bootstrap" strategy="afterInteractive">
      {GA_BOOTSTRAP}
    </Script>
  );
}
