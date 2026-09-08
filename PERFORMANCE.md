| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.1 ± 0.2 | 2.0 | 4.5 | 1.44 ± 0.26 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.5 ± 0.2 | 1.2 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.1 ± 0.0 | 2.0 | 2.4 | 1.33 ± 0.20 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.6 ± 0.2 | 1.2 | 2.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.1 ± 0.1 | 2.0 | 3.1 | 1.35 ± 0.20 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.5 ± 0.2 | 1.1 | 2.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 895.2 ± 25.5 | 846.1 | 959.0 | 8.14 ± 0.27 |
| `cat /home/runner/work/jql/jql/performance_tmp/large.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 110.0 ± 1.8 | 107.5 | 119.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 723.8 ± 29.1 | 675.2 | 794.5 | 5.71 ± 0.24 |
| `cat /home/runner/work/jql/jql/performance_tmp/multi.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 126.7 ± 1.6 | 123.9 | 130.4 | 1.00 |

