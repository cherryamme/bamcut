bamcut 是一个用于从 BAM 文件中提取指定区域的序列并生成 FASTQ 文件的工具。它可以根据提供的染色体区域，从 BAM 文件中截取覆盖该区域的 reads，并将其序列和质量值输出。

## 特性

- 提取指定基因组区域的 reads
- 支持生成压缩的 FASTQ 文件
- 基于 Rust 实现，性能高效

## 安装

确保已安装 Rust 环境，然后克隆仓库并构建：

```bash
git clone https://github.com/cherryamme/bamcut.git
cd bamcut
cargo build --release
```

可执行文件位于 

bamcut

。

## 使用方法

```bash
bamcut -i <input.bam> -o <output.fq.gz> -r <chrom:start-end>
```

- `-i`：输入的 BAM 文件路径
- `-o`：输出的 FASTQ 文件路径（支持 `.gz` 压缩）
- `-r`：指定提取的基因组区域，格式为 `chr:start-end`

## 示例

从 BAM 文件中提取 `chr16:178332-178532` 区域的 reads：

```bash
bamcut -i /path/to/input.bam -o /path/to/output.fq.gz -r chr16:178332-178532
```

## 依赖

- Rust 1.XX 及以上版本
- rust-htslib
- flate2
- log

## 许可证

MIT License
