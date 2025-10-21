class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.12"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.12/git-shade-v1.0.12-x86_64-apple-darwin.tar.gz"
      sha256 "dc7e54cb30a9a31225d515d932dc4dda3a3c19d4fe73a1c9ad7c542ae419ed9b"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.12/git-shade-v1.0.12-aarch64-apple-darwin.tar.gz"
      sha256 "0e51a1bea3cdbaa11c1197ddc1d2b308ff680ffef3d3bc6a35982ee434a5089c"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
