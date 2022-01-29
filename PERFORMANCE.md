| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 36.6 ± 1.5 | 31.1 | 45.2 | 16.91 ± 1.97 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.2 ± 0.2 | 1.6 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 36.4 ± 1.7 | 30.6 | 53.8 | 17.55 ± 2.12 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.1 ± 0.2 | 1.5 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 36.3 ± 1.6 | 30.7 | 44.4 | 17.16 ± 2.25 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.3 | 1.5 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 185.3 ± 4.3 | 168.8 | 203.8 | 1.32 ± 0.05 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 140.7 ± 3.8 | 126.6 | 169.3 | 1.00 |

