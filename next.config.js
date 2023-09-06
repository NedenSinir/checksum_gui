/** @type {import('next').NextConfig} */
const nextConfig = {
   
    output: 'export',
    experimental: {
        appDir: true,
        serverActions:true,
      },
      reactStrictMode:false,
      swcMinify: true,
}

module.exports = nextConfig
