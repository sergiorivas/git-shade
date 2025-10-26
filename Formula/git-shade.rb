class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.13"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.13/git-shade-v1.0.13-x86_64-apple-darwin.tar.gz"
      sha256 "461bdfe370f34b64c30a85a153385fd3a87fa14dcb2b1b5cec97eb2290c0857b"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.13/git-shade-v1.0.13-aarch64-apple-darwin.tar.gz"
      sha256 "c13b3e290b3da1f8defb1cc517ad1c9498f9ca3628dc36a8ab9c4e19053ce910"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
